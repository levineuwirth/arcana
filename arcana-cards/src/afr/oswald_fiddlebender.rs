//! Oswald Fiddlebender — `{1}{W}` 2/2 white Legendary Gnome Artificer.
//! "Magical Tinkering — {W}, {T}, Sacrifice an artifact: Search your library
//! for an artifact card with mana value equal to 1 plus the sacrificed
//! artifact's mana value, put it onto the battlefield, then shuffle."
//!
//! GAP: "sacrifice an artifact" (non-self) cost and "mana value = 1 + sacrificed
//! artifact's mana value" filter — not expressible.

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
    let name = reg.interner_mut().intern("Oswald Fiddlebender");
    let gnome = reg.interner_mut().intern("Gnome");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                text: "{W}, {T}, Sacrifice an artifact: Search your library for an artifact with mana value = 1 + sacrificed's mana value.".into(),
                // GAP: "sacrifice an artifact" (non-self) not expressible.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").unwrap(),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: magical_tinkering,
            }),
    )
}

fn magical_tinkering(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "mana value = 1 + sacrificed artifact's mana value" not computable.
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        tapped: false,
    }]
}
