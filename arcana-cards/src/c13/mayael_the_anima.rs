//! Mayael the Anima — `{R}{G}{W}` 2/3 Legendary Elf Shaman.
//! `{3}{R}{G}{W}, {T}: Look at the top five cards of your library.
//! You may put a creature card with power 5 or greater from among them onto the battlefield.
//! Put the rest on the bottom of your library in any order.`
//! GAP: "look at top 5, conditionally put one onto battlefield, rest to bottom in any order" —
//! TutorToBattlefield is closest but doesn't model "from top 5 only" / "rest to bottom".

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
    let name = reg.interner_mut().intern("Mayael the Anima");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}{G}{W}, {T}: Look at the top five cards of your library. You may put a creature card with power 5 or greater from among them onto the battlefield. Put the rest on the bottom of your library in any order.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}{G}{W}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mayael_ability,
            }),
    )
}

fn mayael_ability(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top 5, put a power 5+ creature onto battlefield, rest to bottom in any order" —
    // no Effect for look-at-top-N with conditional ETB from among them.
    // Using TutorToBattlefield as best-effort (doesn't limit to top 5).
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature().with_min_power(5),
        tapped: false,
    }]
}
