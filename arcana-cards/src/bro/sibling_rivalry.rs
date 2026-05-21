//! Sibling Rivalry — `{3}{R}` sorcery. "Gain control of target
//! artifact or creature until end of turn. Untap it. It gains haste
//! until end of turn. Create a tapped Powerstone token." Threaten-
//! style temporary control isn't expressible; emit Untap+Haste and the
//! Powerstone token; GAP the gain-control duration.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sibling Rivalry");
    let _powerstone = reg.interner_mut().intern("Powerstone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Gain control of target artifact or creature until end of turn. Untap it. It gains haste until end of turn. Create a tapped Powerstone token.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types_any(
                            TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                        ),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // GAP: "gain control until end of turn" — no temporary control variant in catalog; emitting Untap + Haste only.
    // GAP: token cannot be created "tapped" via plain CreateToken (no tapped field).
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let id = *id;
    let powerstone_name = reg.interner().lookup("Powerstone").expect("Powerstone interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(powerstone_name);
    let token = TokenDefinition {
        name: powerstone_name,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::Untap { target: id },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
