//! Boom Scholar — `{1}{R}{G}` 3/3 Goblin Advisor.
//! "Exhaust abilities of other permanents you control cost {2} less to
//! activate." Exhaust — {4}{R}{G}: Creatures and Vehicles you control gain
//! trample until end of turn. Put two +1/+1 counters on this creature.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boom Scholar");
    let goblin = reg.interner_mut().intern("Goblin");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Exhaust is not in the usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };
    // GAP: "Exhaust abilities of other permanents you control cost {2} less" is a
    // static cost-reduction with no expressible primitive.
    reg.register(
        CardDefinition::new(name, chars)
            // Exhaust — {4}{R}{G}: trample to your creatures + two counters on self.
            // GAP: "Vehicles you control" (non-creature half) — only creatures
            // covered; "activate only once" (Exhaust) has no cost field.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Exhaust — {4}{R}{G}: Creatures and Vehicles you control gain trample until end of turn. Put two +1/+1 counters on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{R}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exhaust_trample,
            }),
    )
}

fn exhaust_trample(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::GrantKeyword {
                target: NULL_OBJECT_ID,
                keyword: KeywordAbility::Trample,
                duration: Duration::EndOfTurn,
            }),
        },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        },
    ]
}
