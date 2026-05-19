//! Excavation Explosion — `{2}{R}` sorcery. Excavation Explosion deals 3
//! damage to any target. Create a tapped Powerstone token.
//!
//! GAP: Powerstone token has a "{T}: Add {C}. This mana can't be spent to
//! cast a nonartifact spell." activated ability not expressible in
//! TokenDefinition. Token enters tapped (no tapped field on CreateToken).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Excavation Explosion");
    let _powerstone = reg.interner_mut().intern("Powerstone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Excavation Explosion deals 3 damage to any target. Create a tapped Powerstone token.".into(),
            target_requirements: vec![TargetRequirement::any_target()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    let powerstone = reg.interner().lookup("Powerstone").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(powerstone);
    let token = TokenDefinition {
        name: powerstone,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: token enters tapped (no tapped field on CreateToken)
    // GAP: Powerstone activated ability not expressible in TokenDefinition
    vec![
        Effect::DealDamage { source: entry.source, target: dt, amount: 3 },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
