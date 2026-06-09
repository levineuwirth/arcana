//! Sagittars' Volley — `{2}{G}` instant. Destroy target creature with
//! flying. Deals 1 damage to each creature with flying your opponents
//! control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sagittars' Volley");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature with flying. Sagittars' Volley deals 1 damage to each creature with flying your opponents control.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut effects = vec![Effect::DestroyPermanent { target: *id }];
    let opp_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .with_keyword(KeywordAbility::Flying)
            .controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    for cid in opp_creatures {
        if cid == *id { continue; }
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(cid),
            amount: 1,
        });
    }
    effects
}
