//! Adeline, Resplendent Cathar — `{1}{W}{W}` */4 Legendary Creature —
//! Human Knight with Vigilance.
//!
//! Oracle:
//! * Vigilance.
//! * "Adeline's power is equal to the number of creatures you control."
//!   — a characteristic-defining ability; power is marked `*`
//!   (PtValue::Star). GAP: the CDA is a continuous static, not a
//!   triggered/activated ability.
//! * "Whenever you attack, for each opponent, create a 1/1 white Human
//!   creature token that's tapped and attacking that player or a
//!   planeswalker they control." — modeled on `SelfAttacks` (Adeline
//!   has Vigilance and is the iconic attacker). One token per opponent.
//!   GAP: "Whenever you attack" is a player-attack trigger (no
//!   PlayerAttacks variant); approximated with SelfAttacks. The
//!   per-opponent attack-target assignment is the engine's default
//!   tapped-and-attacking placement.

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
    let name = reg.interner_mut().intern("Adeline, Resplendent Cathar");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let _human_tok = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: power is a CDA "equal to the number of creatures you
        // control" (continuous static; marked `*`).
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "Whenever you attack" (player attack) approximated as
            // SelfAttacks — no PlayerAttacks trigger variant.
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: tokens_per_opponent,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn tokens_per_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human = reg
        .interner()
        .lookup("Human")
        .expect("Human interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    let opponents = script::opponents(state, trig.controller);
    opponents
        .into_iter()
        .map(|_| Effect::CreateTokenTappedAttacking {
            controller: trig.controller,
            token: TokenDefinition {
                name: human,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        })
        .collect()
}
