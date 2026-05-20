//! Toxin Analysis — `{B}` instant. "Target creature gains deathtouch
//! and lifelink until end of turn. Investigate."
//!
//! GAP: catalog has no Effect::Investigate and Pump grants multiple
//! keywords in one bundle (so we use it); the Clue token is created
//! as a plain artifact (its activated 'sac: draw a card' ability
//! isn't expressible in TokenDefinition).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Toxin Analysis");
    let _clue = reg.interner_mut().intern("Clue");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gains deathtouch and lifelink until end of turn. Investigate.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    let clue = reg.interner().lookup("Clue").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(clue);
    let token = TokenDefinition {
        name: clue,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        // GAP: '{2}, Sacrifice: draw a card' activated ability not encoded.
        abilities: vec![],
    };
    vec![
        Effect::Pump {
            target: *id,
            power: 0,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Lifelink],
        },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
