//! Riveteers Ascendancy — `{B}{R}{G}` enchantment (Streets of New Capenna).
//! "Whenever you sacrifice a creature, you may return target creature card
//! with lesser mana value from your graveyard to the battlefield tapped.
//! Do this only once each turn."
//!
//! A `Sacrificed` trigger at `OncePerTurn`. GAPs: the "lesser mana value
//! than the sacrificed creature" target constraint (dynamic relative-cmc
//! filter), the "you may" optionality, and the "tapped" rider.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Riveteers Ascendancy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::creature(),
                },
                intervening_if: None,
                effect: return_lesser_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: vec![TargetRequirement {
                    // GAP: "with lesser mana value" (than the sacrificed
                    // creature) is a dynamic relative-cmc constraint no
                    // ObjectFilter can express; the filter admits any
                    // creature card in the graveyard.
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…you may return target creature card … to the battlefield tapped."
fn return_lesser_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "you may" (resolution-time optionality) and the "tapped" entry
    // rider are not expressible — the return resolves unconditionally and
    // the creature enters untapped.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
