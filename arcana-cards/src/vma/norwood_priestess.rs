//! Norwood Priestess — `{2}{G}{G}` 1/1 Elf Druid.
//! `{T}: You may put a green creature card from your hand onto the battlefield.
//! Activate only during your turn, before attackers are declared.`
//! The "during your turn, before attackers are declared" window is
//! enforced via `ActivationCost.activation_condition`
//! (`conditions::your_turn_before_attackers`).

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
    let name = reg.interner_mut().intern("Norwood Priestess");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: You may put a green creature card from your hand onto the battlefield. Activate only during your turn, before attackers are declared.".into(),
                cost: ActivationCost {
                    // "Activate only during your turn, before attackers
                    // are declared" (CR 602.5e window).
                    activation_condition: Some(|s, _src, you, _reg| {
                        arcana_core::conditions::your_turn_before_attackers(s, you)
                    }),
                    ..ActivationCost::tap_only()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: play_green_creature,
            }),
    )
}

fn play_green_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may put a green creature card from your hand onto the battlefield"
    // — optional pick over the controller's hand.
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature().with_colors(ColorSet::green()),
        tapped: false,
    }]
}
