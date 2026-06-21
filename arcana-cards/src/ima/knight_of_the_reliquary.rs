//! Knight of the Reliquary — `{1}{G}{W}` 2/2 Human Knight.
//!
//! Rules text:
//! * This creature gets +1/+1 for each land card in your graveyard. (static — GAP)
//! * {T}, Sacrifice a Forest or Plains: Search your library for a land card, put
//!   it onto the battlefield, then shuffle.
//!
//! The tutor activated ability is faithful: the cost taps this creature and
//! sacrifices a chosen Forest or Plains you control; the effect puts a land card
//! from your library onto the battlefield (shuffle is automatic). The dynamic
//! self-buff "+1/+1 for each land card in your graveyard" is a static
//! characteristic-defining ability with no demonstrated expression — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of the Reliquary");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let forest = reg.interner_mut().intern("Forest");
    let plains = reg.interner_mut().intern("Plains");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP (static): "gets +1/+1 for each land card in your graveyard" — a
        //       dynamic self CDA with no demonstrated static expression.
        ..Default::default()
    };

    let sac_filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![forest, plains]);

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice a Forest or Plains: Search your library for a land card, put it onto the battlefield, then shuffle.".into(),
            cost: ActivationCost {
                tap: true,
                sacrifice_other: Some(sac_filter),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tutor_land,
        }),
    )
}

fn tutor_land(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        tapped: false,
    }]
}
