//! Ith, High Arcanist — `{5}{W}{U}` 3/5 Legendary Creature — Human Wizard
//! (white/blue) with Vigilance.
//!
//! * Vigilance (keyword line).
//! * `{T}: Untap target attacking creature. Prevent all combat damage that
//!   would be dealt to and dealt by that creature this turn.` — untap and
//!   prevent-all-damage-TO the creature are wired (combat-only fidelity gap);
//!   the "dealt BY that creature" prevention can't be source-keyed to a
//!   single object, so that half is GAP'd.
//! * Suspend 4 — not in the supported KeywordAbility surface. GAP.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ith, High Arcanist");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: Suspend 4—{W}{U} (Suspend not in keyword surface).
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Untap target attacking creature. Prevent all combat damage that would be dealt to and dealt by that creature this turn."
                .into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature().attacking_only()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: untap_and_prevent,
        }),
    )
}

fn untap_and_prevent(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "prevent all combat damage dealt BY that creature" — prevention
    // can't be keyed to a single source object.
    vec![
        Effect::Untap { target: *id },
        Effect::PreventDamage {
            target: DamageTarget::Object(*id),
            amount: None,
            duration: ReplacementDuration::EndOfTurn,
        },
    ]
}
