//! Sally Pride, Lioness Leader — `{3}{W}{W}` 2/4 Legendary Cat Mutant Rebel.
//!
//! Oracle:
//! * When Sally Pride enters, create X 2/2 red Mutant creature tokens, where X
//!   is the number of nontoken creatures you control.
//! * Whenever Sally Pride attacks, put a +1/+1 counter on each creature you
//!   control.
//!
//! The ETB token count is dynamic (`script::count_matching` over nontoken
//! creatures you control), repeating one `CreateToken` per match. The attack
//! trigger fans a +1/+1 counter over each creature you control via
//! `Effect::ForEach` + `Effect::AddCounters`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::TokenDefinition;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sally Pride, Lioness Leader");
    let cat = reg.interner_mut().intern("Cat");
    let mutant = reg.interner_mut().intern("Mutant");
    let rebel = reg.interner_mut().intern("Rebel");
    // Token subtype (Mutant) pre-interned for the ETB token.
    let _mutant_token = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(mutant);
    subtypes.0.insert(rebel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_mutants,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_create_mutants(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // X = number of nontoken creatures you control.
    let n = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .nontoken(),
        trig.controller,
    );
    let mutant = reg.interner().lookup("Mutant").unwrap_or_default();
    let mut token_subs = SubtypeSet::default();
    token_subs.0.insert(mutant);
    let token = TokenDefinition {
        name: mutant,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subs,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}

fn attack_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Put a +1/+1 counter on each creature you control.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: arcana_core::objects::NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}
