//! Involuntary Employment — `{3}{R}` sorcery. "Gain control of target
//! creature until end of turn. Untap that creature. It gains haste
//! until end of turn. Create a Treasure token." Temporary control
//! change is not modeled; we untap + grant haste to the target and
//! create a vanilla Treasure artifact token.

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
    let name = reg.interner_mut().intern("Involuntary Employment");
    let _treasure = reg.interner_mut().intern("Treasure");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Gain control of target creature until end of turn. Untap that creature. It gains haste until end of turn. Create a Treasure token.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: temporary "gain control until end of turn" not modeled.
    let mut out = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        out.push(Effect::Untap { target: *id });
        out.push(Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        });
    }
    let treasure = reg.interner().lookup("Treasure").expect("Treasure interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treasure);
    let token = TokenDefinition {
        name: treasure,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    out.push(Effect::CreateToken { controller: entry.controller, token });
    out
}
