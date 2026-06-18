//! Urborg Panther — `{2}{B}` 2/2 Nightstalker Cat.
//! "{B}, Sacrifice this creature: Destroy target creature blocking it.
//! Sacrifice a creature named Feral Shadow, a creature named
//! Breathstealer, and this creature: Search your library for a card
//! named Spirit of the Night, put that card onto the battlefield, then
//! shuffle."
//!
//! Ability 1: mana + sacrifice-self cost, destroys a target creature.
//! The "blocking it" restriction has no expressible target filter in the
//! documented set, so the filter is a plain target creature (partial).
//! Ability 2: the cost is a multi-named-creature composite sacrifice
//! (three distinct named permanents) that ActivationCost cannot express;
//! GAP'd entirely.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urborg Panther");
    let nightstalker = reg.interner_mut().intern("Nightstalker");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightstalker);
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: second ability — "Sacrifice a creature named Feral Shadow, a
    // creature named Breathstealer, and this creature: ..." is a composite
    // multi-named-permanent sacrifice cost not expressible via ActivationCost.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{B}, Sacrifice this creature: Destroy target creature blocking it.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            // Partial: "blocking it" restriction unexpressible; plain target creature.
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: destroy_blocker,
        }),
    )
}

fn destroy_blocker(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}
