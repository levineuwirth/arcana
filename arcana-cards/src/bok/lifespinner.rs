//! Lifespinner — `{3}{G}` 3/3 Spirit.
//! `{T}, Sacrifice three Spirits: Search your library for a legendary Spirit permanent card,
//! put it onto the battlefield, then shuffle.`
//! GAP: ActivationCost has no "sacrifice three Spirits" field (only sacrifice self);
//! TutorToBattlefield filter for "legendary Spirit" requires legendary+Spirit filter.

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
    let name = reg.interner_mut().intern("Lifespinner");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice three Spirits: Search your library for a legendary Spirit permanent card, put it onto the battlefield, then shuffle.".into(),
                cost: ActivationCost {
                    tap: true,
                    // GAP: no "sacrifice three Spirits" cost field; only sacrifice self
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_legendary_spirit,
            }),
    )
}

fn tutor_legendary_spirit(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ActivationCost has no "sacrifice three Spirits" field.
    // Using TutorToBattlefield with creature filter + legendary supertype as best-effort.
    let filter = ObjectFilter::creature()
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter,
        tapped: false,
    }]
}
