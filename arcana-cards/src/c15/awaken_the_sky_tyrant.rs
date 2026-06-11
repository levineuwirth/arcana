//! Awaken the Sky Tyrant — `{3}{R}` enchantment.
//! "When a source an opponent controls deals damage to you, sacrifice this
//! enchantment. If you do, create a 5/5 red Dragon creature token with
//! flying."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Awaken the Sky Tyrant");
    let _dragon = reg.interner_mut().intern("Dragon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "a SOURCE an opponent controls" includes
                // spells; the permanent-scoped filter is the closest
                // expressible condition.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::Opponent),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: awaken,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…sacrifice this enchantment. If you do, create a 5/5 red Dragon
/// creature token with flying."
fn awaken(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    if p != trig.controller {
        // "deals damage to YOU" — ignore damage to other players.
        return Vec::new();
    }
    let self_name = reg.interner().lookup("Awaken the Sky Tyrant");
    let dragon = reg.interner().lookup("Dragon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    // GAP: fidelity — "If you do" (token contingent on the sacrifice
    // actually happening) is approximated as an unconditional sequence; the
    // sacrifice selects this enchantment by name filter.
    vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter {
                name: self_name,
                ..ObjectFilter::default()
            },
            count: 1,
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: dragon,
                colors: ColorSet::red(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(5)),
                toughness: Some(PtValue::Fixed(5)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        },
    ]
}
