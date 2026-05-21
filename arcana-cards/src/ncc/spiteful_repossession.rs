//! Spiteful Repossession — `{4}{R}` sorcery. "Spiteful Repossession
//! deals damage to each opponent who controls more lands than you
//! equal to the difference. Then create a number of Treasure tokens
//! equal to the damage dealt this way." Per-opponent dynamic amount
//! using script helpers; Treasure tokens emit as artifact shells
//! (activated ability GAPped).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spiteful Repossession");
    let _treasure = reg.interner_mut().intern("Treasure");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Spiteful Repossession deals damage to each opponent who controls more lands than you equal to the difference. Then create a number of Treasure tokens equal to the damage dealt this way.".into(),
                target_requirements: vec![],
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
    let your_lands = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let treasure = reg
        .interner()
        .lookup("Treasure")
        .expect("Treasure interned during register()");
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
    let mut effects: Vec<Effect> = Vec::new();
    let mut total_damage: u32 = 0;
    for opp in script::opponents(state, entry.controller) {
        let opp_lands = script::count_matching(
            state,
            &ObjectFilter::permanent()
                .with_types(TypeLine::LAND.into())
                .controlled_by(ControllerConstraint::You),
            opp,
        );
        if opp_lands > your_lands {
            let diff = opp_lands - your_lands;
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Player(opp),
                amount: diff,
            });
            total_damage += diff;
        }
    }
    for _ in 0..total_damage {
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        });
    }
    effects
}
