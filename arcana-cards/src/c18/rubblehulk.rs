//! Rubblehulk — `{4}{R}{G}` */* Elemental.
//! "Rubblehulk's power and toughness are each equal to the number of
//!  lands you control." (a characteristic-defining static — P/T set to
//!  `*`; the dynamic value is a GAP.)
//! "Bloodrush — {1}{R}{G}, Discard this card: Target attacking creature
//!  gets +X/+X until end of turn, where X is the number of lands you
//!  control." (activated from hand via discard.)

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rubblehulk");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let attacking_creature = ObjectFilter::creature().attacking_only();

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: CDA "power and toughness equal to lands you control" — the
        // dynamic value is a static; P/T transcribed as `*`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Bloodrush — {1}{R}{G}, Discard this card: Target attacking creature gets +X/+X until end of turn, where X is the number of lands you control."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{R}{G}").expect("valid cost"),
                discard_self: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(attacking_creature),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Hand,
            is_instant_speed: false,
            face_gate: None,
            effect: bloodrush_pump,
        }),
    )
}

fn bloodrush_pump(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let x = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    ) as i32;
    vec![Effect::Pump {
        target: *id,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
