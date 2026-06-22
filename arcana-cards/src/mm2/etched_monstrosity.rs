//! Etched Monstrosity — `{5}` 10/10 colorless Artifact Creature —
//! Phyrexian Golem. "This creature enters with five -1/-1 counters on
//! it. {W}{U}{B}{R}{G}, Remove five -1/-1 counters from this creature:
//! Target player draws three cards."
//!
//! GAP: "enters with five -1/-1 counters" — a static enters-with
//! replacement (CR 121.6a); not expressible via a triggered/activated
//! ability. The activated draw ability IS implemented, with the
//! counter removal as its activation cost.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Etched Monstrosity");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}{U}{B}{R}{G}, Remove five -1/-1 counters from \
                       Etched Monstrosity: Target player draws three cards."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}{U}{B}{R}{G}").unwrap(),
                    remove_self_counter: Some((CounterKind::MinusOneMinusOne, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_three,
            }),
    )
}

fn draw_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::DrawCards {
        player: *p,
        count: 3,
    }]
}
