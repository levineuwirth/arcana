//! Promise of Bunrei — `{2}{W}` enchantment.
//! "When a creature you control dies, sacrifice this enchantment. If you
//! do, create four 1/1 colorless Spirit creature tokens."
//!
//! The self-sacrifice uses the name-filtered `Effect::Sacrifice` idiom.
//! // GAP: fidelity — the "if you do" linkage between the sacrifice and
//! // the token creation is not enforced (both effects are sequenced).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Promise of Bunrei");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: sac_and_spawn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…sacrifice this enchantment. If you do, create four 1/1 colorless
/// Spirit creature tokens."
fn sac_and_spawn(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").unwrap_or_default();
    let token = |spirit| {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(spirit);
        TokenDefinition {
            name: spirit,
            colors: ColorSet::new(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        }
    };
    let mut effects = vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter {
            name: reg.interner().lookup("Promise of Bunrei"),
            ..ObjectFilter::default()
        },
        count: 1,
    }];
    for _ in 0..4 {
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: token(spirit),
        });
    }
    effects
}
