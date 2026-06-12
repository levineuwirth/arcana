//! Blood Curdle — `{3}{B}` instant. "Destroy target creature. Put a
//! menace counter on a creature you control." The menace counter is
//! wired as `CounterKind::Named("menace")` plus a permanent Menace
//! grant (CR 122.1g) on a deterministic lowest-id creature you control
//! (the recipient is a resolution-time choice, not a target).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blood Curdle");
    // Interned for the effect fn's lookup of the named counter kind.
    reg.interner_mut().intern("menace");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. Put a menace counter on a creature you control.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut effects = vec![Effect::DestroyPermanent { target: *id }];
    // "a creature you control" is a resolution-time choice, not a target —
    // deterministic lowest-id pick. Menace counter + the keyword it grants
    // (CR 122.1g), modeled as a permanent grant; narrowed GAP: removing the
    // counter later would not revoke the keyword.
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let pick = script::ids_matching(state, &filter, entry.controller)
        .into_iter()
        .min();
    if let (Some(mine), Some(kind)) =
        (pick, reg.interner().lookup("menace").map(CounterKind::Named))
    {
        effects.push(Effect::AddCounters { target: mine, kind, count: 1 });
        effects.push(Effect::GrantKeyword {
            target: mine,
            keyword: KeywordAbility::Menace,
            duration: Duration::Permanent,
        });
    }
    effects
}
