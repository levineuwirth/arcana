//! Rendmaw, Creaking Nest — `{3}{B}{G}` 5/5 Legendary Artifact Creature —
//! Scarecrow. Menace, reach.
//! When Rendmaw enters AND whenever you play a card with two or more card types,
//! each player creates a tapped 2/2 black Bird with flying, goaded for the rest
//! of the game.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rendmaw, Creaking Nest");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let _bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: each_player_birds,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        // GAP: "whenever you play a card with two or more card types" — there is
        // no ObjectFilter predicate for "two or more card types", so the cast
        // half of the combined trigger isn't expressible.
    )
}

fn each_player_birds(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the created Bird tokens enter "tapped" and are "goaded for the rest of
    // the game"; CreateToken has no tapped flag and goad needs a known id, so the
    // tapped + goad riders aren't expressible. Mint the 2/2 black flying Birds.
    let bird = reg.interner().lookup("Bird").unwrap_or_default();
    script::all_players(state)
        .into_iter()
        .map(|p| {
            let mut subs = SubtypeSet::default();
            subs.0.insert(bird);
            Effect::CreateToken {
                controller: p,
                token: TokenDefinition {
                    name: bird,
                    colors: ColorSet::black(),
                    types: TypeLine::CREATURE.into(),
                    subtypes: subs,
                    power: Some(PtValue::Fixed(2)),
                    toughness: Some(PtValue::Fixed(2)),
                    keywords: vec![KeywordAbility::Flying],
                    abilities: vec![],
                },
            }
        })
        .collect()
}
