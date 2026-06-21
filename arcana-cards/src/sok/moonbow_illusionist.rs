//! Moonbow Illusionist — `{2}{U}` 2/1 Moonfolk Wizard.
//! Flying.
//! {2}, Return a land you control to its owner's hand: Target land
//! becomes the basic land type of your choice until end of turn.
//!
//! The keyword (Flying) is a base characteristic. The activated ability's
//! effect — changing a land's basic land type — has no expressible
//! primitive (no SetLandType / SetSubtype effect), so its body is a GAP.
//! The activation cost (mana + return a land you control to hand) is
//! likewise not expressible: the cost catalog has no "return another
//! permanent you control to hand" cost field, only sacrifice/discard/tap.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moonbow Illusionist");
    let moonfolk = reg.interner_mut().intern("Moonfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moonfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP: the activation cost "Return a land you control to its
            // owner's hand" has no ActivationCost field (only
            // sacrifice/discard/tap/life). Modeled as the {2} mana portion
            // only — the bounce-a-land cost is omitted.
            text: "{2}, Return a land you control to its owner's hand: Target land \
                   becomes the basic land type of your choice until end of turn."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: become_basic_land_type,
        }),
    )
}

fn become_basic_land_type(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes the basic land type of your choice" — no effect
    // primitive to set/replace a land's basic land subtype.
    Vec::new()
}
