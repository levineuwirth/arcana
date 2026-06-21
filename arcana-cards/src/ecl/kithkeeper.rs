//! Kithkeeper — `{6}{W}` 3/3 Elemental.
//!
//! Vivid — When this creature enters, create X 1/1 green and white
//! Kithkin creature tokens, where X is the number of colors among
//! permanents you control.
//! Tap three untapped creatures you control: This creature gets +3/+0
//! and gains flying until end of turn.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kithkeeper");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let _ = reg.interner_mut().intern("Kithkin");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: vivid_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap three untapped creatures you control: This creature gets +3/+0 and gains flying until end of turn."
                    .into(),
                cost: ActivationCost {
                    tap_other: Some(ObjectFilter::creature().controlled_by(ControllerConstraint::You)),
                    tap_other_count: 3,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_and_fly,
            }),
    )
}

fn vivid_tokens(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // X = number of colors among permanents you control.
    let colors = [
        ColorSet::white(),
        ColorSet::blue(),
        ColorSet::black(),
        ColorSet::red(),
        ColorSet::green(),
    ];
    let mut x = 0u32;
    for c in colors {
        let f = ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .with_colors(c);
        if script::count_matching(state, &f, trig.controller) > 0 {
            x += 1;
        }
    }

    let kithkin = reg.interner().lookup("Kithkin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);

    let mut effects = Vec::new();
    for _ in 0..x {
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: kithkin,
                colors: ColorSet::green() | ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    effects
}

fn pump_and_fly(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 3,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Flying],
    }]
}
