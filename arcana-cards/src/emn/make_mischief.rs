//! Make Mischief — `{2}{R}` sorcery. "Make Mischief deals 1 damage to
//! any target. Create a 1/1 red Devil creature token. It has 'When
//! this token dies, it deals 1 damage to any target.'"

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Make Mischief");
    let _devil = reg.interner_mut().intern("Devil");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Make Mischief deals 1 damage to any target. Create a 1/1 red Devil creature token. It has \"When this token dies, it deals 1 damage to any target.\"".into(),
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
    let devil = reg.interner().lookup("Devil")
        .expect("Devil interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);
    // NOTE: the token's "when it dies, deal 1 damage" triggered ability
    // is not expressible on a TokenDefinition (abilities vec only holds
    // none here) — that rider is a GAP. The token itself is created.
    let token = TokenDefinition {
        name: devil,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = Vec::new();
    if let Some(target) = entry.targets.targets.first() {
        let dt = match target {
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::Player(p) => DamageTarget::Player(*p),
            TargetChoice::ObjectOrPlayer(o) => match o {
                ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
            },
        };
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: dt,
            amount: 1,
        });
    }
    effects.push(Effect::CreateToken { controller: entry.controller, token });
    effects
}
