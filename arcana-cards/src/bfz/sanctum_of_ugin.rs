//! Sanctum of Ugin — nonbasic land (Battle for Zendikar, 2015).
//! "{T}: Add {C}." and "Whenever you cast a colorless spell with mana
//! value 7 or greater, you may sacrifice this land. If you do, search
//! your library for a colorless creature card, reveal it, put it into
//! your hand, then shuffle." The cast trigger is wired; its effect is
//! a GAP: "you may sacrifice this land. If you do, ..." is an optional
//! SACRIFICE gate, and OptionalPaymentKind supports only Mana / Life
//! (sacrifice gates are documented GAP material).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sanctum of Ugin");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    // Colorless = no colored pips: exclude all five
                    // colors; mana value 7 or greater via min-cmc.
                    filter: Some(
                        ObjectFilter::new()
                            .without_colors(
                                ColorSet::white()
                                    | ColorSet::blue()
                                    | ColorSet::black()
                                    | ColorSet::red()
                                    | ColorSet::green(),
                            )
                            .with_min_cmc(7),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: sac_to_tutor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn sac_to_tutor(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice this land. If you do, search your
    // library for a colorless creature card ..." — the optional
    // sacrifice gate is not expressible (OptionalPaymentKind is only
    // Mana / Life; sacrifice gates are documented GAP material).
    Vec::new()
}
