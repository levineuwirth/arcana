//! Pippin, Warden of Isengard — `{B}{G}` legendary 2/2 Halfling Advisor.
//! `{1}, {T}: Create a Food token.`
//! `{T}, Sacrifice four Foods: Other creatures you control get +3/+3 and gain
//! haste until end of turn. Activate only as a sorcery.`
//! Partner with Merry and the four-Food sacrifice cost are not expressible.

use arcana_core::effects::Effect;
use arcana_core::effects::CommodityToken;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pippin, Warden of Isengard");
    let halfling = reg.interner_mut().intern("Halfling");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Partner with Merry, Warden of Isengard" — Partner is not an
    // available KeywordAbility, and the optional may-tutor-by-name-into-hand
    // ETB has no expressible "target player MAY search" primitive.
    // GAP: "{T}, Sacrifice four Foods: Other creatures you control get +3/+3 and
    // gain haste until end of turn." — sacrifice_other has no count field, so
    // "Sacrifice four Foods" cannot be expressed as an activation cost.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, {T}: Create a Food token.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_food,
        }),
    )
}

fn make_food(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: CommodityToken::Food,
        count: 1,
    }]
}
