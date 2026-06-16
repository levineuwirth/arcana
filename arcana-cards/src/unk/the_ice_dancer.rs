//! The Ice Dancer — `{R}{W}{U}` 2/2 Legendary Creature — Human Athlete (R/U/W).
//!
//! Oracle text:
//! * `{2}, {Q}: The Ice Dancer gains flying until end of turn.`
//!   GAP'd below — the `{Q}` (untap-self) cost has no `ActivationCost` field.
//! * "Whenever The Ice Dancer deals combat damage to a player, create a Gold
//!   token. Then you draw X cards and gain X life, where X is the number of
//!   artifact tokens you control. The Ice Dancer deals X damage to each
//!   opponent."
//!
//! GAP: "{2}, {Q}: gains flying" — {Q} (untap-self) cost has no ActivationCost field

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Ice Dancer");
    let human = reg.interner_mut().intern("Human");
    let athlete = reg.interner_mut().intern("Athlete");
    // Pre-intern the Gold token's name + subtype so the resolver can recover them.
    let _gold = reg.interner_mut().intern("Gold");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(athlete);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}{U}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: on_combat_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "Create a Gold token. Then you draw X cards and gain X life, where X is the
/// number of artifact tokens you control. The Ice Dancer deals X damage to each
/// opponent."
fn on_combat_damage(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP-partial: Gold token's sacrifice-for-mana ability not wired.
    let gold_name = reg.interner().lookup("Gold").unwrap_or_default();
    let mut gold_subtypes = SubtypeSet::default();
    if let Some(sym) = reg.interner().lookup("Gold") {
        gold_subtypes.0.insert(sym);
    }
    let gold_token = TokenDefinition {
        name: gold_name,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: gold_subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };

    // X = number of artifact tokens you control.
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You)
            .tokens_only(),
        trig.controller,
    );

    let mut effects = vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: gold_token,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: n,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: n,
        },
    ];

    for p in script::opponents(state, trig.controller) {
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: n,
        });
    }

    effects
}
