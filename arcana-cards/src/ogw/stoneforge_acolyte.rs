//! Stoneforge Acolyte — `{W}` 1/2 white Kor Artificer Ally. "Cohort — {T}, Tap an
//! untapped Ally you control: Look at the top four cards of your library. You may
//! reveal an Equipment card from among them and put it into your hand. Put the rest
//! on the bottom of your library in any order."
//! GAP: Cohort (tap another Ally you control as cost) and "look at top 4, put
//! Equipment to hand, rest to bottom" search pattern are not modeled.

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
    let name = reg.interner_mut().intern("Stoneforge Acolyte");
    let kor = reg.interner_mut().intern("Kor");
    let artificer = reg.interner_mut().intern("Artificer");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(artificer);
    subtypes.0.insert(ally);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Tap an untapped Ally you control: Look at the top four cards of your library. You may reveal an Equipment card from among them and put it into your hand.".into(),
                cost: ActivationCost {
                    tap: true,
                    // GAP: Cohort (tap another Ally you control) not in ActivationCost
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: cohort_tutor,
            }),
    )
}

fn cohort_tutor(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Look at top 4, reveal Equipment to hand, rest to bottom" is not
    // modeled. TutorToHand searches the whole library without the top-4 constraint.
    Vec::new()
}
