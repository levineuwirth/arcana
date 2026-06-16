//! Ormos, Archive Keeper — `{4}{U}{U}` 5/5 Legendary Sphinx with Flying.
//! Empty-library draw replacement puts five +1/+1 counters on it instead;
//! `{1}{U}{U}, Discard three cards with different names: Draw five cards.`

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Ormos, Archive Keeper");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "If you would draw a card while your library has no cards in it,
    // instead put five +1/+1 counters on Ormos" — a draw-replacement effect,
    // not a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP: "three cards with DIFFERENT NAMES" name-distinctness rider is
            // not expressible; cost models discarding three cards.
            text: "{1}{U}{U}, Discard three cards with different names: Draw five cards.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{U}{U}").expect("valid cost"),
                discard_other: Some(ObjectFilter::default()),
                discard_other_count: 3,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_five,
        }),
    )
}

fn draw_five(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 5,
    }]
}
