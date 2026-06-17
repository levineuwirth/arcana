//! A-Acererak the Archlich — `{2}{B}` 5/5 Legendary Zombie Wizard.
//!
//! Oracle:
//! * Venture into the dungeon (keyword) — NOT a `KeywordAbility` variant
//!   in the demonstrated set, so it is GAP'd (`keywords: vec![]`).
//! * "When Acererak enters, if you haven't completed Tomb of
//!   Annihilation, return it to its owner's hand and venture into the
//!   dungeon." — wired as an ETB trigger (ReturnToHand self + Venture).
//!   The intervening-if "if you haven't completed Tomb of Annihilation"
//!   has no condition helper, so it is GAP'd (`intervening_if: None`).
//! * "Whenever Acererak attacks, create a number of 2/2 black Zombie
//!   creature tokens equal to the number of opponents you have." —
//!   wired with a dynamic count via `script::opponents`.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("A-Acererak the Archlich");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: "Venture into the dungeon" keyword — not a KeywordAbility
        // variant in the demonstrated set.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: intervening-if "if you haven't completed Tomb of
                // Annihilation" — no dungeon-completion condition helper.
                intervening_if: None,
                effect: etb_bounce_and_venture,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_make_zombies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_bounce_and_venture(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::ReturnToHand { target: trig.source },
        Effect::Venture { player: trig.controller },
    ]
}

fn attack_make_zombies(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let n = script::opponents(state, trig.controller).len();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let mut effects = Vec::new();
    for _ in 0..n {
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: zombie,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    effects
}
