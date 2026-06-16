//! Knight-Captain of Eos — `{4}{W}` 2/2 Human Knight.
//! "When this creature enters, create two 1/1 white Soldier creature
//!  tokens. {W}, Sacrifice a Soldier: Prevent all combat damage that
//!  would be dealt this turn."
//!
//! One ETB trigger (two 1/1 white Soldier tokens) and one activated
//! ability ({W} + sacrifice a Soldier you control). The prevention is
//! modeled board-wide (every source to every target this turn).
//! FIDELITY GAP: `PreventDamageFrom` has no combat-only flag, so this
//! prevents ALL damage this turn rather than only combat damage.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight-Captain of Eos");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let sac_soldier = ObjectFilter::permanent().with_subtype_sym(soldier);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_soldiers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}, Sacrifice a Soldier: Prevent all combat damage that would be dealt this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    sacrifice_other: Some(sac_soldier),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_combat_damage,
            }),
    )
}

fn make_soldiers(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    let token = TokenDefinition {
        name: soldier,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}

fn prevent_combat_damage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // FIDELITY GAP: board-wide prevention has no combat-only flag, so this
    // prevents all damage this turn from any source to any target.
    vec![
        Effect::PreventDamageFrom {
            source_filter: ObjectFilter::permanent(),
            target_filter: TargetFilter::Player,
            amount: None,
            duration: ReplacementDuration::EndOfTurn,
        },
        Effect::PreventDamageFrom {
            source_filter: ObjectFilter::permanent(),
            target_filter: TargetFilter::Creature,
            amount: None,
            duration: ReplacementDuration::EndOfTurn,
        },
    ]
}
