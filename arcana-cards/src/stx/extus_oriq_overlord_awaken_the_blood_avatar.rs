//! Extus, Oriq Overlord // Awaken the Blood Avatar — modal DFC (MDFC).
//! Front (Extus, Oriq Overlord — Legendary Creature — Human Warlock, W/B, 2/4):
//!   Double strike.
//!   Magecraft — Whenever you cast or copy an instant or sorcery spell, return
//!     target nonlegendary creature card from your graveyard to your hand.
//! Back (Awaken the Blood Avatar — Sorcery, B/R, {3}{R}{R}{B}):
//!   As an additional cost to cast this spell, you may sacrifice any number of
//!     creatures. This spell costs {2} less to cast for each creature sacrificed.
//!   Each opponent sacrifices a creature of their choice. Create a 3/6 black and
//!     red Avatar creature token with haste and "Whenever this token attacks, it
//!     deals 3 damage to each opponent."
//!
//! GAPs:
//! - Magecraft "or copy a spell" half: only the cast event is wired
//!   (TriggerCondition::SpellCast); a copy of an instant/sorcery does not fire
//!   the trigger.
//! - Back face additional cost "sacrifice any number of creatures; costs {2}
//!   less each": variable sacrifice-for-cost-reduction at cast time is not
//!   expressible as a cost field; not modeled.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Extus, Oriq Overlord");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);
    // Intern the token subtype so the resolver can look it up.
    let _avatar = reg.interner_mut().intern("Avatar");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    // Back face: Awaken the Blood Avatar — Sorcery (B/R).
    let back_name = reg.interner_mut().intern("Awaken the Blood Avatar");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: Some(ManaCost::parse("{3}{R}{R}{B}").expect("valid cost")),
            colors: ColorSet::red() | ColorSet::black(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        },
        spell_ability: Some(SpellAbilityDef {
            text: "Each opponent sacrifices a creature of their choice. Create a 3/6 black \
                   and red Avatar creature token with haste and \"Whenever this token attacks, \
                   it deals 3 damage to each opponent.\""
                .to_string(),
            target_requirements: Vec::new(),
            modal: None,
            effect: awaken_resolve,
        }),
    };

    // Magecraft target: nonlegendary creature card in your graveyard.
    let nonleg_creature_in_gy = TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::creature()
                .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
        },
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // Front (face 0): Magecraft — on cast of instant/sorcery, return
            //   target nonlegendary creature card from graveyard to hand.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: magecraft_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![nonleg_creature_in_gy],
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn magecraft_return(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}

fn awaken_resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let avatar = reg
        .interner()
        .lookup("Avatar")
        .expect("Avatar interned during register()");
    let mut effects: Vec<Effect> = Vec::new();

    // Each opponent sacrifices a creature of their choice.
    for opp in script::opponents(state, entry.controller) {
        effects.push(Effect::Sacrifice {
            player: opp,
            filter: ObjectFilter::creature(),
            count: 1,
        });
    }

    // The token's attack trigger: deals 3 damage to each opponent.
    let attack_trigger = TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::SelfAttacks,
        intervening_if: None,
        effect: avatar_attacks,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    };

    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(avatar);
    let token = TokenDefinition {
        name: avatar,
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Haste],
        abilities: vec![attack_trigger],
    };
    effects.push(Effect::CreateToken {
        controller: entry.controller,
        token,
    });

    effects
}

fn avatar_attacks(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::DealDamage {
            target: DamageTarget::Player(opp),
            amount: 3,
            source: trig.source,
        })
        .collect()
}
