//! Bloody Betrayal — `{2}{R}` sorcery. "Gain control of target creature
//! until end of turn. Untap that creature. It gains haste until end of turn.
//! Create a Blood token." No Effect for temporary gain-control; we emit the
//! untap and haste grant we can. Blood token activated ability isn't
//! expressible — GAP that part.

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
    let name = reg.interner_mut().intern("Bloody Betrayal");
    let _blood = reg.interner_mut().intern("Blood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Gain control of target creature until end of turn. Untap that creature. It gains haste until end of turn. Create a Blood token.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let mut out = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        // GAP: no Effect for "gain control of target permanent until end of turn".
        out.push(Effect::Untap { target: *id });
        out.push(Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        });
    }
    let blood = reg.interner().lookup("Blood").expect("Blood interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(blood);
    // GAP: Blood token's activated ability "{1}, {T}, discard a card, sacrifice: draw"
    // is not expressible — token is created as a bare artifact stub.
    out.push(Effect::CreateToken {
        controller: entry.controller,
        token: TokenDefinition {
            name: blood,
            colors: ColorSet::new(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    });
    out
}
