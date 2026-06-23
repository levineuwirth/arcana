//! Nirkana Revenant — `{4}{B}{B}` 4/4 black Vampire Shade.
//!
//! Oracle:
//! * Whenever you tap a Swamp for mana, add an additional {B}.
//! * `{B}:` This creature gets +1/+1 until end of turn.
//!
//! Implemented:
//! * Tap-a-Swamp trigger — `BecomesTapped { filter: Swamp you control }`,
//!   adds {B}. NOTE: the "for mana" qualifier isn't separately expressible;
//!   the trigger fires on any tap of a Swamp you control.
//! * The `{B}: +1/+1` pump activation.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nirkana Revenant");
    let vampire = reg.interner_mut().intern("Vampire");
    let shade = reg.interner_mut().intern("Shade");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(shade);

    let swamp_filter = script::subtype_filter(reg, "Swamp")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // NOTE: "for mana" qualifier not separately expressible.
                trigger_condition: TriggerCondition::BecomesTapped { filter: swamp_filter },
                intervening_if: None,
                effect: add_additional_black,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}: This creature gets +1/+1 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            }),
    )
}

fn add_additional_black(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, trig.source)],
    }]
}

fn pump_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
