//! Kangee's Lieutenant — `{2}{W}` 1/1 Bird Soldier with Flying.
//!
//! Oracle:
//! * Flying — keyword line.
//! * Whenever this creature attacks, attacking creatures with flying get
//!   +1/+1 until end of turn. — SelfAttacks trigger; pumps every
//!   attacking creature with flying via `ForEach` over the matching ids.
//! * Encore {5}{W} — modeled as a graveyard-activated ability (exile this
//!   card from your graveyard, pay {5}{W}); the per-opponent token-copy
//!   body is GAP'd (not expressible). Encore is not a usable
//!   KeywordAbility variant.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kangee's Lieutenant");
    let bird = reg.interner_mut().intern("Bird");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: "Encore" is not a usable KeywordAbility variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_attacking_flyers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Encore {5}{W} ({5}{W}, Exile this card from your graveyard: \
                       For each opponent, create a token copy that attacks that opponent \
                       this turn if able. They gain haste. Sacrifice them at the beginning \
                       of the next end step. Activate only as a sorcery.)"
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{W}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: encore_effect,
            }),
    )
}

fn pump_attacking_flyers(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .attacking_only()
            .with_keyword(KeywordAbility::Flying),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}

fn encore_effect(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Encore body — per-opponent token copies that attack that opponent,
    // gain haste, and are sacrificed at the next end step is not expressible.
    Vec::new()
}
