//! Bovine Intervention — `{1}{W}` instant. "Destroy target artifact or creature. Its controller
//! creates a 2/2 white Ox creature token."
//! GAP: "its controller" (the controller of the destroyed permanent) creates the token — the
//! catalog uses entry.controller for token creation; looking up the permanent's controller at
//! resolution time is not directly expressible. Modeled with entry.controller as a best effort.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bovine Intervention");
    let _ox = reg.interner_mut().intern("Ox");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target artifact or creature. Its controller creates a 2/2 white Ox creature token.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine::ARTIFACT.into())
                            .with_types_any(TypeLine::CREATURE.into()),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let ox = reg.interner().lookup("Ox").expect("Ox interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ox);
    let token = TokenDefinition {
        name: ox,
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: token should be created under the destroyed permanent's controller, not entry.controller
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
