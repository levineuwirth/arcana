//! Recruitment Officer — `{W}` 2/1 white Human Soldier.
//! "{3}{W}: Look at the top four cards of your library. You may reveal a creature card with
//! mana value 3 or less from among them and put it into your hand. Put the rest on the
//! bottom of your library in a random order."
//! GAP: "look at top 4, reveal creature with mv ≤ 3 to hand" — no Effect variant for
//! top-N look with conditional reveal.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Recruitment Officer");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W}: Look at the top four cards of your library. You may reveal a creature card with mana value 3 or less from among them and put it into your hand. Put the rest on the bottom of your library in a random order.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: look_top_four_recruit,
            }),
    )
}

fn look_top_four_recruit(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top 4, reveal creature with mv ≤ 3 to hand" — no Effect variant
    // for conditional top-N look with reveal. Approximating with TutorToHand.
    let filter = ObjectFilter::creature().with_max_cmc(3);
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter,
        reveal: true,
    }]
}
