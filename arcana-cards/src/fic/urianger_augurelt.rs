//! Urianger Augurelt — `{W}{U}` Legendary 1/3 Elf Advisor.
//!
//! Rules text:
//! * Whenever you play a land from exile or cast a spell from exile, you gain
//!   2 life.
//! * Draw Arcanum — {T}: Look at the top card of your library. You may exile it
//!   face down.
//! * Play Arcanum — {T}: Until end of turn, you may play cards exiled with
//!   Urianger Augurelt. Spells you cast this way cost {2} less to cast.
//!
//! All three abilities are GAP'd. There is no trigger condition for "play/cast
//! from exile". "Look at the top card; you may exile it face down" has no
//! demonstrated effect (exile-face-down-linked-to-source). "Until end of turn
//! you may play cards exiled with ~ for {2} less" needs an exile-linked play
//! permission + cost reduction with no demonstrated hook. The closest available
//! trigger (you cast a spell) would over-fire, so its effect is GAP'd to a
//! no-op; the activations carry their oracle text with empty effects.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urianger Augurelt");
    let elf = reg.interner_mut().intern("Elf");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — no condition for "play a land / cast a spell FROM
                //      EXILE"; using the closest (you cast a spell) but the effect
                //      is GAP'd to avoid over-firing on every spell.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: gain_two_on_play_from_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Draw Arcanum — {T}: Look at the top card of your library. You may exile it face down.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_arcanum,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Play Arcanum — {T}: Until end of turn, you may play cards exiled with Urianger Augurelt. Spells you cast this way cost {2} less to cast.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: play_arcanum,
            }),
    )
}

fn gain_two_on_play_from_exile(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cannot distinguish "from exile" from the SpellCast event; effect omitted
    //      to avoid gaining 2 life on every spell.
    Vec::new()
}

fn draw_arcanum(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at top card; you may exile it face down" — no demonstrated
    //      effect for exile-face-down linked to this source.
    Vec::new()
}

fn play_arcanum(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may play cards exiled with ~ for {2} less until end of turn" —
    //      needs exile-linked play permission + cost reduction; no hook.
    Vec::new()
}
