//! Rat Out — `{B}` instant, "Up to one target creature gets -1/-1
//! until end of turn. You create a 1/1 black Rat creature token with
//! \"This token can't block.\"" The token's can't-block ability is not
//! expressible via the demonstrated TokenDefinition surface.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Rat Out");
    let _rat = reg.interner_mut().intern("Rat");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Up to one target creature gets -1/-1 until end of turn. \
                   You create a 1/1 black Rat creature token with \"This \
                   token can't block.\""
                .into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::UpTo(1),
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
    let rat = reg
        .interner()
        .lookup("Rat")
        .expect("Rat interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    let token = TokenDefinition {
        name: rat,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        // GAP: "This token can't block" cannot be attached via the
        // demonstrated TokenDefinition.abilities surface.
        abilities: vec![],
    };
    let mut effects = vec![Effect::CreateToken {
        controller: entry.controller,
        token,
    }];
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        effects.insert(
            0,
            Effect::Pump {
                target: *id,
                power: -1,
                toughness: -1,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            },
        );
    }
    effects
}
