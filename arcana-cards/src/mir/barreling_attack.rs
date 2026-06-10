//! Barreling Attack — `{2}{R}{R}` instant. "Target creature gains
//! trample until end of turn. When that creature becomes blocked this
//! turn, it gets +1/+1 until end of turn for each creature blocking
//! it."
//!
//! The trample grant is expressible directly. The "becomes-blocked"
//! rider is wired via [`Effect::GrantTriggeredAbility`]: the resolver
//! grants the target a `SelfBecomesBlocked` triggered ability until end
//! of turn whose effect pumps it +N/+N where N counts its blockers via
//! `script::blockers_of` at trigger resolution. (A prior GAP claimed no
//! delayed becomes-blocked primitive existed — stale since the granted-
//! trigger primitive landed.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barreling Attack");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gains trample until end of turn. When that creature becomes blocked this turn, it gets +1/+1 until end of turn for each creature blocking it.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // "When that creature becomes blocked this turn, it gets +1/+1 until
    // end of turn for each creature blocking it" — granted triggered
    // ability; the EndOfTurn grant duration carries the "this turn" rider.
    let ability = TriggeredAbilityDef {
        id: GRANTED_TRIGGER_ID_BASE + 1,
        trigger_condition: TriggerCondition::SelfBecomesBlocked,
        intervening_if: None,
        effect: granted_blocked_pump,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    };
    vec![
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantTriggeredAbility {
            target: *id,
            ability: Box::new(ability),
            duration: Duration::EndOfTurn,
        },
    ]
}

fn granted_blocked_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::blockers_of(state, trig.source).len() as i32;
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::Pump {
        target: trig.source,
        power: n,
        toughness: n,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
