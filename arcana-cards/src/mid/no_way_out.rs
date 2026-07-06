//! No Way Out — `{2}{B}` sorcery. "Target opponent discards two cards.
//! You create a 2/2 black Zombie creature token with decayed." Decayed
//! (can't block + sac-on-attack) isn't an evergreen KeywordAbility, so
//! the token can't carry it. We use `CreateTokenSacEot` to model the
//! 'sacrifice it at end of combat' shape (closest available — actual
//! decayed sacrifices on attack/end-of-combat; this is an approximation
//! to the next-end-step sacrifice the catalog provides).

use arcana_core::effects::{DiscardChoice, Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("No Way Out");
    let _zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent discards two cards. You create a 2/2 black Zombie creature token with decayed.".into(),
                target_requirements: vec![TargetRequirement::target_opponent()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let target_p = *p;
    let _ = script::all_players(state);
    let zombie = reg
        .interner()
        .lookup("Zombie")
        .expect("Zombie interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: 'decayed' (can't-block + sac-on-attack) — modeled via the
    // sac-at-EOT helper as the closest catalog primitive.
    vec![
        Effect::Discard {
            player: target_p,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::CreateTokenSacEot {
            controller: entry.controller,
            token,
        },
    ]
}
