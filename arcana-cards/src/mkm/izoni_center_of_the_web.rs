//! Izoni, Center of the Web — `{4}{B}{G}` 5/4 Legendary Elf Detective with
//! Menace.
//! "Whenever Izoni enters or attacks, you may collect evidence 4. If you do,
//! create two 2/1 black and green Spider creature tokens with menace and
//! reach."
//! "Sacrifice four tokens: Surveil 2, then draw two cards. You gain 2 life."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Izoni, Center of the Web");
    let elf = reg.interner_mut().intern("Elf");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever Izoni enters or attacks" — two trigger defs.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: collect_evidence_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: collect_evidence_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice four tokens: Surveil 2, then draw two cards. You gain 2 life.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(ObjectFilter::permanent().tokens_only()),
                    sacrifice_other_count: 4,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: surveil_draw_gain,
            }),
    )
}

fn collect_evidence_tokens(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may collect evidence 4. If you do, create two ... tokens" —
    // "collect evidence N" (exile cards with total mana value ≥ N from your
    // graveyard) is not an OptionalPayment cost kind, so the may-pay gate that
    // produces the tokens cannot be expressed; whole effect omitted to avoid
    // unconditional token creation. (Both enters/attacks triggers are wired.)
    Vec::new()
}

fn surveil_draw_gain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Surveil {
            player: ctx.controller,
            count: 2,
        },
        Effect::DrawCards {
            player: ctx.controller,
            count: 2,
        },
        Effect::GainLife {
            player: ctx.controller,
            amount: 2,
        },
    ]
}
