//! Grip of Phyresis — `{2}{U}` instant. "Gain control of target
//! Equipment, then create a 0/0 black Phyrexian Germ creature token
//! and attach that Equipment to it." 'Attach an Equipment to a token'
//! is not in the catalog (no Equip primitive). Best effort: take
//! permanent control of the Equipment and create the Germ token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grip of Phyresis");
    let _phyrexian = reg.interner_mut().intern("Phyrexian");
    let _germ = reg.interner_mut().intern("Germ");
    let _equipment_filter_subtype = reg.interner_mut().intern("Equipment");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Gain control of target Equipment, then create a 0/0 black Phyrexian Germ creature token and attach that Equipment to it.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
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
    let phyrexian = reg
        .interner()
        .lookup("Phyrexian")
        .expect("Phyrexian interned during register()");
    let germ = reg
        .interner()
        .lookup("Germ")
        .expect("Germ interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(germ);
    let token = TokenDefinition {
        name: germ,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: 'Equipment' subtype-only targeting; auto-attach to the Germ.
    vec![
        Effect::ChangeControl {
            target: *id,
            new_controller: entry.controller,
        },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
