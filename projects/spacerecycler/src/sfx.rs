use ggez::audio::SoundSource;
use ggez::Context;
use ggez::GameResult;
use std::fs::File;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
pub struct Sfx {
    pub bullet_data: ggez::audio::SoundData,
    pub bullet_sound: Vec<ggez::audio::Source>,
    pub life_data: ggez::audio::SoundData,
    pub life_sound: Option<ggez::audio::Source>,
    pub asteroid_data: ggez::audio::SoundData,
    pub asteroid_sound: Vec<ggez::audio::Source>,
    pub overload_data: ggez::audio::SoundData,
    pub overload_sound: Vec<ggez::audio::Source>,
    pub collector_data: ggez::audio::SoundData,
    pub collector_sound: Option<ggez::audio::Source>,
    pub collectorw_data: ggez::audio::SoundData,
    pub collectorw1_sound: Option<ggez::audio::Source>,
    pub collectorw2_sound: Option<ggez::audio::Source>,
    pub turbo_data: ggez::audio::SoundData,
    pub turbo_sound: Option<ggez::audio::Source>,
    pub money_data: ggez::audio::SoundData,
    pub money_sound: Option<ggez::audio::Source>,
    pub last_asteroid_sound: Instant,
}

impl Sfx {
    fn load_file(path: &str) -> GameResult<ggez::audio::SoundData> {
        let basepath = std::env::current_dir()?;
        let path = basepath.join(path);
        let mut file = File::open(&path)?;
        ggez::audio::SoundData::from_read(&mut file)
    }
    pub fn new() -> GameResult<Self> {
        Ok(Self {
            bullet_data: Self::load_file("sfx/machinegun_bullet1.ogg")?,
            bullet_sound: vec![],
            life_data: Self::load_file("sfx/life_lost1.ogg")?,
            life_sound: None,
            asteroid_data: Self::load_file("sfx/asteroid_hit1.ogg")?,
            asteroid_sound: vec![],
            overload_data: Self::load_file("sfx/overload1.ogg")?,
            overload_sound: vec![],
            last_asteroid_sound: Instant::now(),
            collector_data: Self::load_file("sfx/collector1.ogg")?,
            collector_sound: None,
            collectorw_data: Self::load_file("sfx/collector_working1.ogg")?,
            collectorw1_sound: None,
            collectorw2_sound: None,
            turbo_data: Self::load_file("sfx/turbo1.ogg")?,
            turbo_sound: None,
            money_data: Self::load_file("sfx/money1.ogg")?,
            money_sound: None,
        })
    }
    pub fn asteroid_hit(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.asteroid_hit_vol(ctx, 0.05)
    }
    pub fn asteroid_hit_vol(&mut self, ctx: &mut Context, vol: f32) -> GameResult<()> {
        if self.last_asteroid_sound.elapsed() < Duration::from_millis(100) {
            return Ok(());
        }
        self.last_asteroid_sound = Instant::now();
        for bs in self.asteroid_sound.iter_mut() {
            if bs.stopped() {
                bs.set_volume(vol);
                bs.play(ctx)?;
                return Ok(());
            }
        }
        let mut bs = ggez::audio::Source::from_data(ctx, self.asteroid_data.clone())?;
        bs.set_volume(vol);
        bs.play(ctx)?;
        self.asteroid_sound.push(bs);
        Ok(())
    }
    pub fn bullet(&mut self, ctx: &mut Context, vol: f32) -> GameResult<()> {
        for bs in self.bullet_sound.iter_mut() {
            if bs.stopped() {
                bs.set_volume(vol);
                bs.play(ctx)?;
                return Ok(());
            }
        }
        let mut bs = ggez::audio::Source::from_data(ctx, self.bullet_data.clone())?;
        bs.set_volume(vol);
        bs.play(ctx)?;
        self.bullet_sound.push(bs);
        Ok(())
    }
    pub fn life_lost(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(ls) = self.life_sound.as_mut() {
            if ls.stopped() {
                ls.play(ctx)?;
            }
            return Ok(());
        }
        let mut ls = ggez::audio::Source::from_data(ctx, self.life_data.clone())?;
        ls.set_volume(0.1);
        ls.play(ctx)?;
        self.life_sound = Some(ls);
        Ok(())
    }
    pub fn turbo(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(ls) = self.turbo_sound.as_mut() {
            ls.play(ctx)?;
            return Ok(());
        }
        let mut ls = ggez::audio::Source::from_data(ctx, self.turbo_data.clone())?;
        ls.set_volume(0.1);
        ls.play(ctx)?;
        self.turbo_sound = Some(ls);
        Ok(())
    }
    pub fn overload(&mut self, ctx: &mut Context) -> GameResult<()> {
        for bs in self.overload_sound.iter_mut() {
            if bs.stopped() {
                bs.play(ctx)?;
                return Ok(());
            }
        }
        let mut bs = ggez::audio::Source::from_data(ctx, self.overload_data.clone())?;
        bs.set_volume(0.1);
        bs.play(ctx)?;
        self.overload_sound.push(bs);
        Ok(())
    }
    pub fn collector(&mut self, ctx: &mut Context, on: bool) -> GameResult<()> {
        if self.collector_sound.is_none() {
            let mut ls = ggez::audio::Source::from_data(ctx, self.collector_data.clone())?;
            ls.set_volume(0.3);
            ls.set_fade_in(Duration::from_millis(200));
            self.collector_sound = Some(ls);
        }
        let ls = self.collector_sound.as_mut().unwrap();
        let vol = ls.volume();
        if !on && ls.playing() {
            if vol > 0.001 {
                ls.set_volume(vol / 1.01);
            } else {
                ls.stop(ctx)?;
            }
        } else if vol < 0.3 {
            ls.set_volume(vol * 1.1 + 0.01);
        }
        if on && !ls.playing() {
            ls.play(ctx)?;
        }
        Ok(())
    }
    pub fn collectorw1(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.collectorw1_sound.is_none() {
            let mut ls = ggez::audio::Source::from_data(ctx, self.collectorw_data.clone())?;
            ls.set_volume(0.4);
            ls.set_pitch(1.2);
            self.collectorw1_sound = Some(ls);
        }
        let ls = self.collectorw1_sound.as_mut().unwrap();
        if !ls.playing() || ls.elapsed() > Duration::from_millis(200) {
            ls.play(ctx)?;
        }

        Ok(())
    }
    pub fn collectorw2(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.collectorw2_sound.is_none() {
            let mut ls = ggez::audio::Source::from_data(ctx, self.collectorw_data.clone())?;
            ls.set_volume(0.8);
            ls.set_pitch(0.8);
            self.collectorw2_sound = Some(ls);
        }
        let ls = self.collectorw2_sound.as_mut().unwrap();
        if !ls.playing() || ls.elapsed() > Duration::from_millis(200) {
            ls.play(ctx)?;
        }
        Ok(())
    }

    pub fn money(&mut self, ctx: &mut Context, diff: f32) -> GameResult<()> {
        if self.money_sound.is_none() {
            let mut ls = ggez::audio::Source::from_data(ctx, self.money_data.clone())?;
            ls.set_volume(0.1);
            self.money_sound = Some(ls);
        }
        let ls = self.money_sound.as_mut().unwrap();
        if !ls.playing()
            || ls.elapsed().as_secs_f32()
                > (0.5 / (diff.abs() / 3.0).max(1.0).sqrt().sqrt()).max(0.1)
        {
            if diff > 1.0 {
                ls.set_pitch(1.414);
            }
            if diff < -1.0 {
                ls.set_pitch(0.707 + diff.abs().log2() / 50.0);
            }
            ls.play(ctx)?;
        }
        Ok(())
    }
}
