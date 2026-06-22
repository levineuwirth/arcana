//! Irascible Wolverine — `{2}{R}` 3/2 red Creature — Wolverine.
//!
//! When this creature enters, exile the top card of your library. Until
//! end of turn, you may play that card.
//! Plot {2}{R}
//!
//! Decomposition: the ETB impulse-exile → one `TriggeredAbilityDef`
//! emitting `Effect::ImpulseExile { count: 1 }`. Plot is NOT in the
//! usable keyword surface (it's a cast-from-exile cost mechanic), so
//! `keywords: vec![]` and the Plot line is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Irascible Wolverine");
    let wolverine = reg.interner_mut().intern("Wolverine");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolverine);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: keyword "Plot {2}{R}" — cast-from-exile cost mechanic, not
        // in the usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_impulse,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_impulse(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 1,
    }]
}
