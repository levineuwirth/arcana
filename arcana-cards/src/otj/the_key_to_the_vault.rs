//! The Key to the Vault — `{1}{U}` Legendary artifact — Equipment.
//! "Whenever equipped creature deals combat damage to a player, look at
//! that many cards from the top of your library. You may exile a nonland
//! card from among them. Put the rest on the bottom of your library in a
//! random order. You may cast the exiled card without paying its mana
//! cost. Equip {2}{U}"
//!
//! // GAP: trigger — "equipped creature" cannot be expressed as a
//! // DamageDealt source filter (no attached-to-source predicate); the
//! // filter is approximated as "a creature you control".
//! // GAP: effect — "exile a chosen nonland card from the looked-at cards
//! // and cast it without paying its mana cost" has no primitive
//! // (DigTopN goes to hand; ImpulseExile exiles all N at normal cost);
//! // the effect is a no-op.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Key to the Vault");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{2}{U}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: should fire only for the EQUIPPED creature; no
                // attached-to-source filter exists, so this approximates
                // with "a creature you control".
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: dig_and_free_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// See module GAP note — the dig + free-cast tail is inexpressible.
fn dig_and_free_cast(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at that many cards, exile a nonland card from among
    // them, cast it without paying its mana cost" not expressible.
    Vec::new()
}
