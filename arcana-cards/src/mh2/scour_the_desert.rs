//! Scour the Desert — `{3}{W}{W}` sorcery. "Exile target creature
//! card from your graveyard. Create X 1/1 white Bird creature tokens
//! with flying, where X is the exiled card's toughness."
//!
//! We read the target card's toughness before exiling it (via
//! `script::toughness_of`), exile it from the graveyard, then mint
//! that many 1/1 white flying Bird tokens (one `CreateToken` per
//! point of toughness — the count is dynamic, so it is computed, not
//! hardcoded).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scour the Desert");
    let _bird = reg.interner_mut().intern("Bird");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target creature card from your graveyard. Create X 1/1 white Bird creature tokens with flying, where X is the exiled card's toughness.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let x = script::toughness_of(state, *id).max(0) as u32;

    let bird = reg.interner().lookup("Bird").expect("Bird interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    let token = TokenDefinition {
        name: bird,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };

    let mut effects = vec![Effect::ExileFromGraveyard { target: *id }];
    for _ in 0..x {
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        });
    }
    effects
}
