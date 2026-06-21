//! Dire Fleet Ravager — `{3}{B}{B}` 4/4 Orc Pirate Wizard with Menace
//! and Deathtouch.
//!
//! "When this creature enters, each player loses a third of their life,
//! rounded up."
//!
//! Menace and Deathtouch are base keywords. The ETB trigger makes each
//! player lose ceil(life/3); per-player amounts are computed from the
//! live state and wrapped in a Sequence.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dire Fleet Ravager");
    let orc = reg.interner_mut().intern("Orc");
    let pirate = reg.interner_mut().intern("Pirate");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(pirate);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_lose_third,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_lose_third(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| {
            let life = script::life(state, p).max(0) as u32;
            // a third rounded up = ceil(life / 3)
            let amount = (life + 2) / 3;
            Effect::LoseLife { player: p, amount }
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
