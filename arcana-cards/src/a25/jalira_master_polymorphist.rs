//! Jalira, Master Polymorphist — `{3}{U}` 2/2 blue Legendary Human Wizard.
//! "{2}{U}, {T}, Sacrifice another creature: Reveal cards from the top of
//! your library until you reveal a nonlegendary creature card. Put that card
//! onto the battlefield and the rest on the bottom of your library in a
//! random order."
//!
//! GAP: "reveal until nonlegendary creature, put on battlefield, rest to
//! bottom in random order" — TutorToBattlefield approximates the "find
//! nonlegendary creature" part; the "rest to bottom in random order" is
//! a GAP.

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
    let name = reg.interner_mut().intern("Jalira, Master Polymorphist");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}, {T}, Sacrifice another creature: Put a nonlegendary creature from your library onto the battlefield.".into(),
                // GAP: cannot sacrifice "another" creature; approximated as mana+tap
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: polymorph_ability,
            }),
    )
}

fn polymorph_ability(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal until nonlegendary creature; rest to bottom in random order"
    // approximated as TutorToBattlefield for nonlegendary creature
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature().without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
        tapped: false,
    }]
}
