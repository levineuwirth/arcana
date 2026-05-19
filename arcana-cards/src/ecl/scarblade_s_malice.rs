//! Scarblade's Malice — `{B}` instant. "Target creature you control gains deathtouch and lifelink
//! until end of turn. When that creature dies this turn, create a 2/2 black and green Elf creature token."
//! GAP: conditional death trigger attached to target creature for this turn only is not expressible
//! with the spell/trigger catalog (would need an until-end-of-turn triggered ability granted to the
//! permanent, not modeled). Pump with keywords is expressed; token creation on death is GAP.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scarblade's Malice");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gains deathtouch and lifelink until end of turn. When that creature dies this turn, create a 2/2 black and green Elf creature token.".into(),
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
    // GAP: "when that creature dies this turn, create a 2/2 black and green Elf token" —
    //      granting a temporary triggered ability to a permanent is not modeled in the catalog.
    vec![
        Effect::Pump {
            target: *id,
            power: 0,
            toughness: 0,
            duration: arcana_core::layers::Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Lifelink],
        },
    ]
}
