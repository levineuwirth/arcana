//! Yes Man, Personal Securitron — `{2}{W}` 2/2 Legendary Artifact Creature — Robot.
//! {T}: Target opponent gains control of Yes Man. When they do, you draw two
//! cards and put a quest counter on Yes Man. Activate only during your turn.
//! Wild Card — When Yes Man leaves the battlefield, its owner creates a tapped
//! 1/1 white Soldier creature token for each quest counter on it.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yes Man, Personal Securitron");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // Wild Card is an ability word (not an expressible KeywordAbility).
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP (rider): "Activate only during your turn." — a timing
            // restriction with no demonstrated cost/legality field.
            text: "{T}: Target opponent gains control of Yes Man. When they do, you draw two cards and put a quest counter on Yes Man.".into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: give_control,
        }),
        // GAP: "Wild Card — When Yes Man leaves the battlefield, its owner
        // creates a TAPPED 1/1 white Soldier token for each quest counter on
        // it." No plain "create tapped token" primitive (only tapped-and-
        // attacking), and the quest-counter count is unreadable once the source
        // has left the battlefield.
    )
}

fn give_control(
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
    // The reflexive "when they do" rider (draw two, quest counter) is folded
    // into resolution here as a Sequence (a minor timing-fidelity gap).
    vec![Effect::Sequence(vec![
        Effect::ChangeControl {
            target: ctx.source,
            new_controller: *p,
        },
        Effect::DrawCards {
            player: ctx.controller,
            count: 2,
        },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Quest,
            count: 1,
        },
    ])]
}
