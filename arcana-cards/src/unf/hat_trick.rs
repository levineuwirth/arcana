//! Hat Trick — `{1}{W}` instant. "Target blocking or blocked creature
//! you control gets +1/+1 and gains first strike until end of turn. If
//! it has a hat, it gains double strike until end of turn. If you and
//! that creature each have a hat, the creature gains triple strike until
//! end of turn."
//!
//! GAP: "has a hat" / "triple strike" checks — hat is not a tracked
//! game concept and triple strike is not a KeywordAbility. Best-effort:
//! +1/+1 and first strike only.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hat Trick");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target blocking or blocked creature you control gets +1/+1 and gains first strike until end of turn. If it has a hat, it gains double strike until end of turn. If you and that creature each have a hat, the creature gains triple strike until end of turn.".into(),
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
    // GAP: "has a hat" check for double/triple strike conditional
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::FirstStrike],
    }]
}
