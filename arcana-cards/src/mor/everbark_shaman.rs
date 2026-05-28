//! Everbark Shaman — `{4}{G}` 3/5 green Treefolk Shaman.
//! "{T}, Exile a Treefolk card from your graveyard: Search your library for
//! up to two Forest cards, put them onto the battlefield tapped, then shuffle."
//!
//! GAP: "Exile a Treefolk card from your graveyard" as an activation cost —
//! ActivationCost cannot exile a specific-type card from the graveyard.
//! Approximated as tap-only with the tutor effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Everbark Shaman");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Exile a Treefolk card from your graveyard: Search your library for up to two Forest cards, put them onto the battlefield tapped, then shuffle.".into(),
                // GAP: exile-treefolk-from-graveyard cost not expressible
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: search_two_forests,
            }),
    )
}

fn search_two_forests(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Tutor two Forests to battlefield tapped
    // Effect::TutorToBattlefield only tutors one; repeat for two
    let forest_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    vec![
        Effect::TutorToBattlefield { player: ctx.controller, filter: forest_filter.clone(), tapped: true },
        Effect::TutorToBattlefield { player: ctx.controller, filter: forest_filter, tapped: true },
    ]
}
