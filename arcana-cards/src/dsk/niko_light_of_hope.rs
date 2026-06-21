//! Niko, Light of Hope — `{2}{W}{U}` 3/4 Legendary Human Wizard.
//! ETB: create two Shard enchantment tokens ("{2}, Sacrifice this
//! token: Scry 1, then draw a card.").
//! `{2}, {T}`: Exile target nonlegendary creature you control; Shards
//! become copies of it until next end step; return it at the next end
//! step.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Niko, Light of Hope");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let _shard = reg.interner_mut().intern("Shard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    // Scryfall-parsed "Scry" belongs to the granted token ability, not Niko's
    // keyword line — there is no `KeywordAbility::Scry`, so the keyword line is
    // empty.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: ETB "create two Shard tokens" — the Shard is an enchantment
            // token whose only function is its granted activated ability
            // ("{2}, Sacrifice: Scry 1, then draw"); a bare enchantment token
            // with no power/toughness and no expressible abilities (token
            // activated abilities aren't part of this API) carries no game
            // effect, so the ETB creation is omitted as inexpressible.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_shards,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {2}, {T}: Exile target nonlegendary creature you control, then
            // return it at the next end step. The "Shards become copies of it"
            // rider is GAP'd (no token-copy-of-exiled linkage here).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}: Exile target nonlegendary creature you control. \
                       Shards you control become copies of it until the next end \
                       step. Return it to the battlefield under its owner's \
                       control at the beginning of the next end step."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You)
                            .without_supertypes(
                                SupertypeSet::new().with(SupertypeSet::LEGENDARY),
                            ),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: blink_own_creature,
            }),
    )
}

fn etb_make_shards(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: enchantment-token creation with a granted "{2}, Sac: Scry 1, draw"
    // activated ability is not expressible in this card class.
    Vec::new()
}

fn blink_own_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "Shards you control become copies of it until the next end step" —
    // no token-copy linkage to the exiled object is expressible.
    // The blink (exile now + return at the next end step) IS faithful.
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::DelayedAction {
            source: *id,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
